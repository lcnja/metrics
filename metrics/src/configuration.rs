use crate::receiver::Receiver;
use std::time::Duration;

/// A configuration builder for [`Receiver`].
#[derive(Clone)]
pub struct Configuration {
    pub(crate) capacity: usize,
    pub(crate) batch_size: usize,
    pub(crate) histogram_window: Duration,
    pub(crate) histogram_granularity: Duration,
}

impl Default for Configuration {
    fn default() -> Configuration {
        Configuration {
            capacity: 512,
            batch_size: 64,
            histogram_window: Duration::from_secs(10),
            histogram_granularity: Duration::from_secs(1),
        }
    }
}

impl Configuration {
    /// Creates a new [`Configuration`] with default values.
    pub fn new() -> Configuration {
        Default::default()
    }

    /// Sets the buffer capacity.
    ///
    /// Defaults to 512.
    ///
    /// This controls the size of the channel used to send metrics.  This channel is shared amongst
    /// all active sinks.  If this channel is full when sending a metric, that send will be blocked
    /// until the channel has free space.
    ///
    /// Tweaking this value allows for a trade-off between low memory consumption and throughput
    /// burst capabilities.  By default, we expect samples to occupy approximately 64 bytes.  Thus,
    /// at our default value, we preallocate roughly ~32KB.
    ///
    /// Generally speaking, sending and processing metrics is fast enough that the default value of
    /// 512 supports millions of samples per second.
    pub fn capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }

    /// Sets the batch size.
    ///
    /// Defaults to 64.
    ///
    /// This controls the size of message batches that we collect for processing.  The only real
    /// reason to tweak this is to control the latency from the sender side.  Larger batches lower
    /// the ingest latency in the face of high metric ingest pressure at the cost of higher ingest
    /// tail latencies.
    ///
    /// Long story short, you shouldn't need to change this, but it's here if you really do.
    pub fn batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Sets the histogram configuration.
    ///
    /// Defaults to a 10 second window with 1 second granularity.
    ///
    /// This controls both how long of a time window we track histogram data for, and the
    /// granularity in which we roll off old data.
    ///
    /// As an example, with the default values, we would keep the last 10 seconds worth of
    /// histogram data, and would remove 1 seconds worth of data at a time as the window rolled
    /// forward.
    pub fn histogram(mut self, window: Duration, granularity: Duration) -> Self {
        self.histogram_window = window;
        self.histogram_granularity = granularity;
        self
    }

    /// Create a [`Receiver`] based on this configuration.
    pub fn build(self) -> Receiver {
        Receiver::from_config(self)
    }
}