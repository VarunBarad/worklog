import csv
import datetime


def main():
    worklog_file = '/Users/varunb/worklog.csv'

    worklog_entries = []
    with open(worklog_file, 'r', newline='') as csvFile:
        reader = csv.DictReader(csvFile)
        worklog_entries.extend(reader)

    print('Enter the new worklog entry')
    new_entry = input()

    now = datetime.datetime.now()
    formatted_now = now.strftime('%Y-%m-%dT%H:%M+05:30')

    worklog_entries.append({
        'timestamp': formatted_now,
        'contents': new_entry,
    })

    with open(worklog_file, 'w', newline='') as csvFile:
        headers = ['timestamp', 'contents']
        writer = csv.DictWriter(csvFile, fieldnames=headers)
        writer.writeheader()
        writer.writerows(worklog_entries)

    print('{} Worklog updated!'.format(now.strftime('%H:%M')))