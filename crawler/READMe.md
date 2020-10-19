# Crawler Overview
Crawler is responsible for providing domain data (schools, catchment areas, mapping etc) for each supported region.
Each supported region has a separate crawler which is responsible to fetch the required data.

# Supported Regions
berlin - Berlin area

# Running crawler
##  Running Locally
      cargo run <region>
          -  Valid region values: berlin

##  Running in K8s as Docker container

## Running tests
```
cargo test
```
This will run both unit and integration tests.

### Unit Tests
Most unit tests go into a tests mod with the #[cfg(test)] attribute. Test functions are marked with the #[test] attribute.
All unit tests are under respective file

### Integration Tests
 Cargo looks for integration tests in tests directory next to src. all integration tests are under /tests directory

# Errors


# Design Choices
Only one crawler is run in a process. For running different region, invoke a separate call
How to store region-specific data for domain objects? For eg: schools in berlin might get more addiitonal data which might not be available or relevant to other regions.
 - for this, we can dump additional data as a JSON.  <key: string, value: object>
For Berlin crawling, currently only simple details flow is designed and implemented. What this means , is that not all the sub nodes of details node are crawled. This is done to keep in mind the MVP requirements. 
How will the running region crawler communicate its state? 
 - For MVP, we keep it simple and log
 Implement extensible logging mechanism
  - For MVP, we log to console



# Learnings 
How to namespace modules and types
macros
Compiler when run using cargo build does not show all errors together.
Debugging 
async functions in trait
dyn keyword in trait


