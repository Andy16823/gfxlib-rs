use std::time::Instant;

use crate::utils;

/// A struct that records and provides frame statistics, such as frame time and delta time.
///
/// `FrameStatsRecorder` is used to track the time taken by each frame, calculate the delta time (time difference between frames),
/// and provide the elapsed time statistics, including the frame time in milliseconds and the delta time in seconds.
pub struct FrameStatsRecorder {
    /// Time at the start of the current frame.
    pub ellapsed_time: Instant,
    /// The ellpased millisecs
    pub ellapsed: u128,
    /// The time difference between the current and previous frames (in seconds).
    pub delta_time: u128,
    /// The time elapsed for the last frame (in milliseconds).
    pub last_frame: u128,
}

impl FrameStatsRecorder {
    /// Creates a new `FrameStatsRecorder` instance.
    ///
    /// This method initializes the recorder with default values. The `ellapsed_time` is set to the current time,
    /// and the `delta_time`, `last_frame_s`, and `last_frame_ms` are initialized to 0.
    ///
    /// # Returns
    /// A new `FrameStatsRecorder` with default values.
    pub fn new() -> FrameStatsRecorder {
        FrameStatsRecorder {
            ellapsed_time: Instant::now(),
            ellapsed: 0,
            delta_time: 0,
            last_frame: 0,
        }
    }

    /// Completes the current frame, calculates the delta time, and updates the frame statistics.
    ///
    /// This method should be called once per frame, ideally at the end of the frame.
    /// It calculates the time elapsed for the current frame and updates the `delta_time`, `last_frame_s`, and `last_frame_ms` fields.
    /// The `ellapsed_time` is reset to the current time, preparing it for the next frame.
    pub fn frame_complete(&mut self) {
        let ellapsed = self.ellapsed_time.elapsed();
        self.ellapsed = ellapsed.as_millis();
        let current_frame_time = utils::current_time_millis();
        self.delta_time = current_frame_time.wrapping_sub(self.last_frame);
        self.last_frame = current_frame_time;
        self.ellapsed_time = Instant::now();
    }

    /// Prints the current frame statistics: frame time (in milliseconds), delta time (in seconds), and FPS.
    ///
    /// This method prints the following information:
    /// - The frame time in milliseconds (`last_frame_ms`)
    /// - The delta time in seconds (`delta_time`)
    /// - The calculated FPS (frames per second), based on the delta time.
    ///
    /// # Example Output
    /// ```
    /// Framestats: frametime: 16ms Deltatime: 0.016s FPS: 62.5
    /// ```
    pub fn print_stats(&self) {
        println!(
            "Framestats: frametime: {}ms deltatime: {}s fps: {}",
            self.ellapsed, self.delta_time, self.fps()
        );
    }

    /// Calculates and returns the current frames per second (FPS).
    ///
    /// FPS is calculated as the inverse of `delta_time` (1 / delta_time). If the delta time is 0.0 (which may happen in certain edge cases),
    /// the FPS will be returned as 0.0.
    ///
    /// # Returns
    /// The FPS, calculated as the reciprocal of `delta_time`. If `delta_time` is 0, returns 0.0.
    pub fn fps(&self) -> u128 {
        if self.delta_time > 0 {
            // Avoid division by zero by checking delta_time
            return 1000 / self.delta_time;
        } else {
            return 0;
        }
    }
}
