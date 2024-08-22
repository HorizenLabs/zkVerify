"""
A script to automatically profile CI runs.
Runs are queried from most recent to least recent and processed only if successfully completed.

Usage examples:
    python3 ci/profile-CI.py --help
    python3 ci/profile-CI.py ghp_0123456789ABCDEF0123456789ABCDEF0123
    python3 ci/profile-CI.py ghp_0123456789ABCDEF0123456789ABCDEF0123 --stop-after-processed 5

NOTE: if you incur in `403 - rate limit exceeded` error, wait some time for refreshment and use parameters
      `--stop-after-processed` and `--skip` to query the runs you need.
"""

import requests
import argparse
from datetime import datetime

REPOSITORY_OWNER = 'HorizenLabs'
REPOSITORY = 'zkVerify'
DATE_FORMAT = "%Y-%m-%dT%H:%M:%SZ"

STOP_AFTER_PROCESSED_DEFAULT = 10
SKIP_DEFAULT = 0
JOBS_TO_PROFILE_DEFAULT = ['build-test-job / build-and-test', 'test-coverage-job / coverage', 'lint-format-job / lint-and-format', 'e2e-test-job / e2e-test']

def parse_arguments():
    """
    Parse command-line arguments and return the parsed arguments.
    """
    parser = argparse.ArgumentParser(description="Profile CI runs")
    parser.add_argument("token", help="GitHub personal access token (for avoiding rate limit threshold)")
    parser.add_argument("--stop-after-processed", nargs='?', type=int, default=STOP_AFTER_PROCESSED_DEFAULT, help="stop after processing N successfully completed workflow runs")
    parser.add_argument("--skip", nargs='?', type=int, default=SKIP_DEFAULT, help="skip first N successfully completed workflow runs")
    parser.add_argument("--jobs-to-profile", nargs='*', default=JOBS_TO_PROFILE_DEFAULT, help="names of the jobs to profile")
    return parser.parse_args()


# MAIN
if __name__ == "__main__":
    # Parse command-line arguments
    args = parse_arguments()

    headers = {
        'Authorization': '{Bearer ' + args.token + '}',
        'Accept': 'application/vnd.github+json'
    }
    url_runs = f'https://api.github.com/repos/{REPOSITORY_OWNER}/{REPOSITORY}/actions/runs'
    response = requests.get(url_runs, headers=headers)
    if response.status_code == 200:
        print(f"Run_Id;Run_Number;Job_Id;Total_Seconds")
        processed = 0
        skipped = 0
        workflow_runs = response.json()
        for run in workflow_runs['workflow_runs']:
            if processed == args.stop_after_processed:
                break
            if run['status'] == 'completed' and run['conclusion'] == 'success':
                if skipped < args.skip:
                    skipped = skipped + 1
                    continue
                processed = processed + 1
                url_run_single = f'https://api.github.com/repos/{REPOSITORY_OWNER}/{REPOSITORY}/actions/runs/{run["id"]}/timing'
                response = requests.get(url_run_single, headers=headers)
                if response.status_code == 200:
                    workflow_run_details = response.json()
                    for job in workflow_run_details['billable']['UBUNTU']['job_runs']:
                        url_job_single = f'https://api.github.com/repos/{REPOSITORY_OWNER}/{REPOSITORY}/actions/jobs/{job["job_id"]}'
                        response = requests.get(url_job_single, headers=headers)
                        if response.status_code == 200:
                            job_run_single = response.json()
                            if (job_run_single['name'] in args.jobs_to_profile):
                                datetime_started = datetime.strptime(job_run_single['started_at'], DATE_FORMAT)
                                datetime_completed = datetime.strptime(job_run_single['completed_at'], DATE_FORMAT)
                                datetime_diff = datetime_completed - datetime_started
                                print(f"{run['id']};{run['run_number']};{job_run_single['name']};{datetime_diff.total_seconds()}")
                        else:
                            print(f"Failed to get job run details: {response.status_code} - {response.reason}")
                else:
                    print(f"Failed to get workflow run details: {response.status_code} - {response.reason}")
    else:
        print(f"Failed to get workflow runs: {response.status_code} - {response.reason}")
