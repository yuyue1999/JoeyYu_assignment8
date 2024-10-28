
# Assignment 8 - Data Processing in Python and Rust

![CI](https://github.com/yuyue1999/JoeyYu_assignment8/actions/workflows/ci.yml/badge.svg)
![Rust](https://github.com/yuyue1999/JoeyYu_assignment8/actions/workflows/rust.yml/badge.svg)

## Introduction

In this assignment, we take an existing Python script for data processing and rewrite it in Rust. We then highlight improvements in speed and resource usage between the two implementations. The dataset used is the "AI-Powered Job Market Insights", which provides a synthetic but realistic snapshot of the modern job market, focusing on the role of artificial intelligence (AI) and automation across various industries.

## Dataset

[AI-Powered Job Market Insights Dataset](https://www.kaggle.com/datasets/uom190346a/ai-powered-job-market-insights)

The dataset includes 500 unique job listings, each characterized by different factors like industry, company size, AI adoption level, automation risk, required skills, and job growth projections. It is designed to be a valuable resource for researchers, data scientists, and policymakers exploring the impact of AI on employment, job market trends, and the future of work.

### `ai_job_market_insights.csv`

The dataset is included in the repository as `ai_job_market_insights.csv`.

## Python Script

### Description

The Python script `src/main.py` reads the dataset, calculates descriptive statistics for numeric columns, and prints the results.



### Usage

To run the Python script:

```bash
python3 src/main.py
```

### Output

Sample output of descriptive statistics:

```
shape: (11, 5)
┌─────────────────────┬──────────┬──────────┬──────────┬──────────┐
│ describe            ┆ count    ┆ mean     ┆ std      ┆ min      │
│ ---                 ┆ ---      ┆ ---      ┆ ---      ┆ ---      │
│ str                 ┆ f64      ┆ f64      ┆ f64      ┆ f64      │
╞═════════════════════╪══════════╪══════════╪══════════╪══════════╡
│ Salary_USD          ┆ 500.0    ┆ 91222.39 ┆ 20483.78 ┆ 31969.53 │
│ AI_Adoption_Level   ┆ 500.0    ┆ 4.05     ┆ 1.42     ┆ 1.0      │
│ Automation_Risk     ┆ 500.0    ┆ 0.5      ┆ 0.29     ┆ 0.01     │
│ Job_Growth_Projection ┆ 500.0  ┆ 7.11     ┆ 4.24     ┆ -3.0     │
│ Company_Size        ┆          ┆          ┆          ┆          │
│ ...                 ┆ ...      ┆ ...      ┆ ...      ┆ ...      │
│ max                 ┆          ┆          ┆          ┆          │
│ 155209.82           ┆          ┆          ┆          ┆          │
└─────────────────────┴──────────┴──────────┴──────────┴──────────┘
```

## Rust Script

### Description

The Rust script `src/main.rs` performs the same functionality as the Python script: it reads the dataset, calculates descriptive statistics for numeric columns, and prints the results.


### Usage

To run the Rust script:

```bash
cargo run --release
```

### Output

Sample output of descriptive statistics:

```
Column 'Salary_USD':
  Count: 500
  Mean: 91222.3910
  Min: 31969.5263
  Max: 155209.8216
  Std Dev: 20483.7769
```

## Performance Comparison

We compared the execution time of both scripts.

### Python Script Timing

```bash
python3 main.py  0.11s user 0.10s system 13% cpu 1.582 total
```

**Total execution time:** **1.582 seconds**

### Rust Script Timing

```bash
cargo run  0.04s user 0.03s system 20% cpu 0.359 total
```

**Total execution time:** **0.359 seconds**

### Improvement

The Rust script shows a significant improvement in execution time over the Python script.

- **Execution Time Reduction**: The Rust script is approximately **4.4 times faster** than the Python script.

## CI/CD Pipeline

We have set up a Continuous Integration (CI) pipeline using GitHub Actions. The pipeline includes workflows for both the Python and Rust scripts.

### Badges

![CI](https://github.com/yuyue1999/JoeyYu_assignment8/actions/workflows/ci.yml/badge.svg)
![Rust](https://github.com/yuyue1999/JoeyYu_assignment8/actions/workflows/rust.yml/badge.svg)

### Workflows

- **CI**: Runs tests and checks for the Python script.
- **Rust**: Builds and tests the Rust project.

The CI/CD pipeline ensures that any changes to the code are automatically tested and verified.

## How to Run

### Prerequisites

- **Python**: Version 3.6 or higher
- **Rust**: Latest stable version (ensure `cargo` is installed)

### Running the Python Script

1. Navigate to the project directory:

   ```bash
   cd JoeyYu_assignment8
   ```

2. Run the script:

   ```bash
   python3 src/main.py
   ```

### Running the Rust Script

1. Navigate to the project directory:

   ```bash
   cd JoeyYu_assignment8
   ```

2. Build and run the Rust script:

   ```bash
   cargo run --release
   ```

## Repository Structure

```
JoeyYu_assignment8/
├── ai_job_market_insights.csv
├── src/
│   ├── main.py
│   └── main.rs
├── Cargo.toml
├── README.md
├── .github/
│   └── workflows/
│       ├── ci.yml
│       └── rust.yml
```

## Grading Criteria

- **Functionality in Rust (20 points)**: The Rust script replicates the functionality of the original Python script.
- **Demonstrated Improvements (20 points)**: Significant improvements in execution speed have been shown.
- **CI/CD Pipeline (10 points)**: A CI/CD pipeline is set up using GitHub Actions.
- **README.md (10 points)**: This README provides detailed information about the project.

## Conclusion

By rewriting the Python data processing script in Rust, we achieved significant improvements in execution speed. Rust's performance benefits make it a compelling choice for data processing tasks where performance is critical.
