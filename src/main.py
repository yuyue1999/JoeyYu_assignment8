import os
import polars as pl
from collections import Counter

def read_dataset(path):
    script_dir = os.path.dirname(os.path.abspath(__file__))
    dataset_path = os.path.join(script_dir, path)
    df = pl.read_csv(dataset_path)
    return df


def statistics(df):
    print(df.describe())

def main():
    dataset_path = "../ai_job_market_insights.csv"
    df = read_dataset(dataset_path)
    statistics(df)


if __name__ == "__main__":
    main()