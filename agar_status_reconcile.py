"""Reconcile the items in index with item timestamps to produce a list of items that need uploading
"""

import datetime
import re
import sys

# column indices in the index file
IDX_ITEM_NAME = 1
IDX_TIMESTAMP = 2


def _main(index_file, timestamps_file):
    timestamp_index = _read_index(index_file)
    with open(timestamps_file) as f:
        for line in f.readlines():
            item_name, timestamp = line.rstrip().split("\t")
            if item_name not in timestamp_index:
                print("a\t%s" % item_name)
            elif _is_after(timestamp, timestamp_index[item_name]):
                print("u\t%s\t[%s > %s]" % (item_name, timestamp, timestamp_index[item_name]))


def _read_index(index_file):
    with open(index_file) as f:
        return {cols[IDX_ITEM_NAME]: cols[IDX_TIMESTAMP]
                for line in f.readlines()
                for cols in [line.rstrip().split("\t")]}


def _is_after(ts1, ts2):
    """Check if ts1 > ts2"""
    return _parse_ts(ts1) > _parse_ts(ts2)    


def _parse_ts(timestamp):
    # Python 2's strptime does not support parsing time zones
    ts_zone = OffsetTimeZone(timestamp.split(" ")[-1])
    ts_parseable_part = re.match(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}\.\d{6}", timestamp).group(0)
    parsed = datetime.datetime.strptime(ts_parseable_part, "%Y-%m-%d %H:%M:%S.%f")
    parsed.replace(tzinfo=ts_zone)
    return parsed


class OffsetTimeZone(datetime.tzinfo):
    """Fixed offset in +/-hhmm from UTC."""

    def __init__(self, offset):
        direction = 1 if offset[0] == '+' else -1
        hours = int(offset[1:3])
        minutes = int(offset[3:])
        offset_minutes = direction * (60 * hours + minutes)
        self.__offset = datetime.timedelta(minutes = offset_minutes)
        self.__name = offset

    def utcoffset(self, dt):
        return self.__offset

    def tzname(self, dt):
        return self.__name

    def dst(self, dt):
        return datetime.timedelta(0)


if __name__ == '__main__':
    _main(sys.argv[1], sys.argv[2])
