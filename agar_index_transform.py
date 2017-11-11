""" Transforms AWS index json into agar's TSV index.
"""
import json
import sys


def _main(json_file_path, tsv_file_path):
    with open(json_file_path) as json_file:
        index_json = json.load(json_file)
    with open(tsv_file_path, 'w') as tsv_file:
        for archive in index_json['ArchiveList']:
            try:
                category, name, timestamp, archive_format, password_reminder = \
                    archive['ArchiveDescription'].split('/')
            except ValueError:
                print("unable to parse archive description '%s', skipping" %
                      archive['ArchiveDescription'])
                continue
            tsv_file.write("%s\t%s\t%s\t%s\t%s\t%s\n" % (category,
                                                         name,
                                                         timestamp,
                                                         archive_format,
                                                         archive['Size'],
                                                         password_reminder))


if __name__ == '__main__':
    _main(sys.argv[1], sys.argv[2])
