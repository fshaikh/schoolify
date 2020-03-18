struct Statistics {
    pub school_count: u64,
    pub catchment_area_count: u64,
    pub school_crawl_time: f64,
    pub catchment_area_crawl_time: f64,
    pub total_crawl_time: f64,
}

impl Default for Statistics {
    fn default() -> Statistics {
        Statistics {
            school_count: 0,
            catchment_area_count: 0,
            school_crawl_time: 0.,
            catchment_area_crawl_time: 0.,
            total_crawl_time: 0.,
        }
    }
}

static mut STATISTICS: Statistics = Statistics {
    school_count: 0,
    catchment_area_count: 0,
    school_crawl_time: 0.,
    catchment_area_crawl_time: 0.,
    total_crawl_time: 0.,
};

pub fn record_school_count(count: u64) {
    unsafe {
        STATISTICS.school_count = count;
    }
}

pub fn record_catchment_area_count(count: u64) {
    unsafe {
        STATISTICS.catchment_area_count = count;
    }
}

pub fn record_school_crawl_time(time: f64) {
    unsafe {
        STATISTICS.school_crawl_time = time;
    }
}

pub fn record_catchment_area_crawl_time(time: f64) {
    unsafe {
        STATISTICS.catchment_area_crawl_time = time;
    }
}

pub fn record_total_crawl_time(time: f64) {
    unsafe {
        STATISTICS.total_crawl_time = time;
    }
}

pub fn get_statistics_formatted() -> String {
    unsafe {
        return format!(
            "
        School count: {}, 
        Catchment Area Count: {},
        School Crawl Time: {},
        Catchment Area Time: {},
        Total Crawl Time: {}
    ",
            STATISTICS.school_count,
            STATISTICS.catchment_area_count,
            STATISTICS.school_crawl_time,
            STATISTICS.catchment_area_crawl_time,
            STATISTICS.total_crawl_time
        );
    }
}
