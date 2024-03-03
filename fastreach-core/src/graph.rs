use std::str::Utf8Error;

use byteorder::{LittleEndian as LE, ReadBytesExt};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use fnv::FnvHashMap;
use geo::{GeoFloat, HaversineDistance};
use num_traits::FromPrimitive;
use smallvec::SmallVec;

const MOVE_SPEED: f32 = 1000.0 / 12.0; // in m/min

pub struct Node<'a> {
    // binary data
    data: &'a [u8],
    /// offset in the file where edge information can be found
    pub outgoing: SmallVec<[Edge<'a>; 4]>,
}

impl<'a> Node<'a> {
    #[must_use]
    pub fn lat(&self) -> f32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            f32::from_le_bytes(self.data[8..12].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn lon(&self) -> f32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            f32::from_le_bytes(self.data[12..16].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn to_point(&self) -> geo::Point<f32> {
        geo::Point::from([self.lon(), self.lat()])
    }

    /// # Errors
    /// When name is not utf-8 encoded.
    pub fn name(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(&self.data[20..])
    }
}

impl<'a> rstar::RTreeObject for Node<'a> {
    type Envelope = rstar::AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        rstar::AABB::from_point([self.lon(), self.lat()])
    }
}

pub struct Edge<'a> {
    data: &'a [u8],
}

impl<'a> Edge<'a> {
    #[must_use]
    pub fn start(&self) -> u32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[..4].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn end(&self) -> u32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[4..8].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn walk(&self) -> u16 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u16::from_le_bytes(self.data[8..10].try_into().unwrap_unchecked())
        }
    }

    pub fn journeys(&self) -> impl Iterator<Item = Journey<'a>> {
        let journey_bytes = unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[10..14].try_into().unwrap_unchecked())
        } as usize;
        let journeys = &self.data[14..14 + journey_bytes];
        journeys.chunks_exact(6).map(|c| Journey { data: c })
    }

    #[must_use]
    pub fn operating_periods(&self) -> OperatingPeriodIter<'a> {
        let journey_bytes = unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[10..14].try_into().unwrap_unchecked())
        } as usize;
        let mut offset = 14 + journey_bytes;
        let period_bytes = unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[offset..offset + 4].try_into().unwrap_unchecked())
        } as usize;
        offset += 4;
        OperatingPeriodIter {
            data: &self.data[offset..offset + period_bytes],
            offset: 0,
        }
    }
}

pub struct Journey<'a> {
    data: &'a [u8],
}

impl<'a> Journey<'a> {
    #[must_use]
    pub fn arrival(&self) -> u16 {
        unsafe {
            // can only error when len of slice is not 2 which panics beforehand
            u16::from_le_bytes(self.data[..2].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn departure(&self) -> u16 {
        unsafe {
            // can only error when len of slice is not 2 which panics beforehand
            u16::from_le_bytes(self.data[2..4].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn operating_period_index(&self) -> u16 {
        unsafe {
            // can only error when len of slice is not 2 which panics beforehand
            u16::from_le_bytes(self.data[4..6].try_into().unwrap_unchecked())
        }
    }
}

pub struct OperatingPeriod<'a> {
    data: &'a [u8],
}

impl<'a> OperatingPeriod<'a> {
    #[must_use]
    pub fn start(&self) -> u32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[..4].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn end(&self) -> u32 {
        unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[4..8].try_into().unwrap_unchecked())
        }
    }

    #[must_use]
    pub fn valid_days(&self) -> &[u8] {
        let len = unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(self.data[8..12].try_into().unwrap_unchecked())
        } as usize;
        &self.data[12..12 + len]
    }
}

pub struct OperatingPeriodIter<'a> {
    offset: usize,
    data: &'a [u8],
}

impl<'a> Iterator for OperatingPeriodIter<'a> {
    type Item = OperatingPeriod<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.offset;
        if start >= self.data.len() {
            return None;
        }
        self.offset += 8;
        let valid_days_len = unsafe {
            // can only error when len of slice is not 4 which panics beforehand
            u32::from_le_bytes(
                self.data[self.offset..self.offset + 4]
                    .try_into()
                    .unwrap_unchecked(),
            )
        };
        self.offset += 4 + valid_days_len as usize;
        Some(OperatingPeriod {
            data: &self.data[start..self.offset],
        })
    }
}

pub struct Graph<'a> {
    pub nodes: Vec<Node<'a>>,
    pub ids: FnvHashMap<u64, usize>,
}

type Error = Box<dyn std::error::Error>;

impl<'a> Graph<'a> {
    /// Parses the given slice into the graph.
    /// # Errors
    /// When file is too small.
    pub fn from_slice(data: &[u8]) -> Result<Graph, Error> {
        let mut reader = std::io::Cursor::new(data);
        let node_count = reader.read_u32::<LE>()?;
        let mut nodes = Vec::with_capacity(node_count as usize);
        let mut ids = FnvHashMap::<u64, usize>::default();
        for i in 0..node_count {
            let start: usize = reader.position().try_into()?;
            let id = reader.read_u64::<LE>()?;
            ids.insert(id, i as usize);
            // 2 * 4 bytes lat and lon
            reader.set_position(reader.position() + 8);
            let name_len = reader.read_u32::<LE>()? as u64;
            reader.set_position(reader.position() + name_len);
            let end: usize = reader.position().try_into()?;
            nodes.push(Node {
                data: &data[start..end],
                outgoing: SmallVec::new(),
            });
        }
        let edge_count = reader.read_u32::<LE>()?;
        for _ in 0..edge_count {
            let offset: usize = reader.position().try_into()?;
            let start = reader.read_u32::<LE>()?;
            // end + walk_seconds 6 bytes
            reader.set_position(reader.position() + 6);
            let journeys_bytes = reader.read_u32::<LE>()?;
            reader.set_position(reader.position() + journeys_bytes as u64);
            let periods_bytes = reader.read_u32::<LE>()?;
            reader.set_position(reader.position() + periods_bytes as u64);
            let end = reader.position().try_into()?;
            nodes[start as usize].outgoing.push(Edge {
                data: &data[offset..end],
            });
        }
        Ok(Graph { nodes, ids })
    }
}

pub struct TimedNode<'a, 'b> {
    pub node: &'a Node<'b>,
    pub duration: chrono::Duration,
}

impl<'a, 'b> TimedNode<'a, 'b> {
    #[must_use]
    pub fn new(node: &'a Node<'b>, duration: chrono::Duration) -> Self {
        TimedNode { node, duration }
    }

    #[must_use]
    pub fn to_points<T: GeoFloat + FromPrimitive>(&self) -> Vec<geo::Coord<T>> {
        let distance = num_traits::cast::<f32, T>(MOVE_SPEED).unwrap()
            * num_traits::cast::<i64, T>(self.duration.num_minutes()).unwrap();
        crate::vincenty::spherical_circle(
            geo::Coord::from((
                num_traits::cast(self.node.lon()).unwrap(),
                num_traits::cast(self.node.lat()).unwrap(),
            )),
            8,
            distance,
        )
    }

    #[must_use]
    pub fn to_poly<T: GeoFloat + FromPrimitive>(&self) -> geo::Polygon<T> {
        let mut verts = self.to_points();
        verts.push(verts[0]);
        let line_string = geo::LineString::new(verts);
        geo::Polygon::new(line_string, vec![])
    }
}

impl<'a, 'b> PartialEq for TimedNode<'a, 'b> {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration
    }
}

impl<'a, 'b> Eq for TimedNode<'a, 'b> {}

impl<'a, 'b> PartialOrd for TimedNode<'a, 'b> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.duration.partial_cmp(&other.duration)
    }
}

impl<'a, 'b> Ord for TimedNode<'a, 'b> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.duration.cmp(&other.duration)
    }
}

impl<'a, 'b> rstar::RTreeObject for &TimedNode<'a, 'b> {
    type Envelope = rstar::AABB<[f32; 2]>;

    fn envelope(&self) -> Self::Envelope {
        self.node.envelope()
    }
}

pub struct IsochroneDijsktra<'a, 'b> {
    graph: &'a Graph<'b>,
    periods: Vec<OperatingPeriod<'b>>,
}

impl<'a, 'b: 'a> IsochroneDijsktra<'a, 'b> {
    #[must_use]
    pub fn new(graph: &'a Graph<'b>) -> Self {
        Self {
            graph,
            periods: Vec::new(),
        }
    }

    fn u16_to_time(number: u16) -> NaiveTime {
        let minute = number % 60;
        let hour = number / 60;
        NaiveTime::from_hms_opt(hour as u32, minute as u32, 0).unwrap()
    }

    fn u32_to_date(number: u32) -> NaiveDate {
        let day = number % 100;
        let month = (number % 10000) / 100;
        let year = (number / 10000) + 2000;
        NaiveDate::from_ymd_opt(year as i32, month, day).unwrap()
    }

    fn valid_on(period: &OperatingPeriod<'b>, date: NaiveDate) -> Result<bool, Error> {
        let start = Self::u32_to_date(period.start());
        let end = Self::u32_to_date(period.end());
        if date < start || date > end {
            return Ok(false);
        }
        let diff = date - start;
        let days: usize = diff.num_days().try_into()?;
        let idx = days / 8;
        let off = days % 8;
        Ok((1 << off & period.valid_days()[idx]) > 0)
    }

    fn next_journey(
        &mut self,
        edge: &Edge<'b>,
        start: NaiveDateTime,
    ) -> Result<Option<Journey<'b>>, Error> {
        let mut departure = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let mut result = None;
        self.periods.extend(edge.operating_periods());
        for journey in edge.journeys() {
            let current_departure = Self::u16_to_time(journey.departure());
            if current_departure > departure || current_departure < start.time() {
                continue;
            }
            let period = &self.periods[journey.operating_period_index() as usize];
            if !Self::valid_on(period, start.date())? {
                continue;
            }
            departure = current_departure;
            result = Some(journey);
        }
        self.periods.clear();
        Ok(result)
    }

    fn get_walk(edge: &Edge<'b>) -> Option<chrono::Duration> {
        let walk = edge.walk();
        if walk == u16::MAX {
            return None;
        }
        Some(chrono::Duration::seconds(walk as i64))
    }

    fn next_journey_duration(
        &mut self,
        edge: &Edge<'b>,
        start: NaiveDateTime,
    ) -> Result<Option<chrono::Duration>, Error> {
        let opt_journey = self.next_journey(edge, start)?;
        if opt_journey.is_none() {
            return Ok(None);
        }
        let journey = opt_journey.unwrap();
        let arrival = Self::u16_to_time(journey.arrival());
        let duration = arrival - start.time();
        if duration >= chrono::Duration::zero() {
            return Ok(Some(duration));
        }
        // travel over midnight
        let midnight = NaiveTime::from_hms_opt(23, 59, 59).unwrap();
        let pre_midnight = midnight - start.time();
        let post_midnight = arrival - NaiveTime::MIN;
        Ok(Some(pre_midnight + post_midnight))
    }

    pub fn nodes_within(
        &mut self,
        node_idx: usize,
        start: NaiveDateTime,
        duration: chrono::Duration,
    ) -> Result<Vec<TimedNode<'a, 'b>>, Error> {
        let mut result: Vec<TimedNode<'a, 'b>> = Vec::new();
        let node = &self.graph.nodes[node_idx];
        result.push(TimedNode::new(node, duration));
        let mut arrivals = FnvHashMap::<u32, NaiveDateTime>::default();
        arrivals.insert(node_idx.try_into()?, start);
        let mut heap = rudac::heap::FibonacciHeap::<TimedNode<'a, 'b>>::init_min();
        let max_time = start + duration;
        heap.push(TimedNode::new(node, chrono::Duration::zero()));
        while let Some(current) = heap.pop() {
            let departure = start + current.duration;
            for out in &current.node.outgoing {
                let opt_walk = Self::get_walk(out);
                let opt_journey = self.next_journey_duration(out, departure)?;
                let out_duration = match (opt_walk, opt_journey) {
                    (None, None) => continue,
                    (None, Some(j)) => j,
                    (Some(w), None) => w,
                    (Some(w), Some(j)) => w.min(j),
                };
                let total_duration = current.duration + out_duration;
                let arrival = start + total_duration;
                if arrival > max_time {
                    continue;
                }
                let stored_arrival = arrivals.get(&out.end()).unwrap_or(&NaiveDateTime::MAX);
                if arrival < *stored_arrival {
                    let out_node = &self.graph.nodes[out.end() as usize];
                    let distance = current
                        .node
                        .to_point()
                        .haversine_distance(&out_node.to_point());
                    let current_radius =
                        MOVE_SPEED * (duration - current.duration).num_minutes() as f32;
                    let out_radius = MOVE_SPEED * (duration - total_duration).num_minutes() as f32;
                    if distance + out_radius > current_radius {
                        result.push(TimedNode::new(out_node, duration - total_duration));
                    }
                    arrivals.insert(out.end(), arrival);
                    heap.push(TimedNode::new(out_node, total_duration));
                }
            }
        }
        Ok(result)
    }
}
