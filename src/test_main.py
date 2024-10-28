import os
from main import statistics, companysize, generate_pdf, read_dataset, jobgrowth, requiredskill

dataset_path = "../ai_job_market_insights.csv"
df = read_dataset(dataset_path)


def _file_exists_and_not_empty(filepath):
    return os.path.exists(filepath) and os.path.getsize(filepath) > 0


def test_stat():
    statistics(df)
