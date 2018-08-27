Agar
====

Agar (Amazon Glacier Archiver) is a set of utilities for backup and archival on
[Amazon Glacier](https://aws.amazon.com/glacier/). Features:

* end-to-end encryption: the data leaves your computer encrypted with a strong
  symmetric cipher (AES256)

System Requirements
-------------------

Agar has only been tested on Linux. Bash and Python are required.

Setup
-----

* clone this repo
* install [aws-cli](https://aws.amazon.com/cli/)
* configure aws-cli: 

~~~
aws configure
~~~

* create a Glacier vault for your backups: 

~~~
aws glacier create-vault --account-id - --vault-name my-backup
~~~

* create `~/.agar/agar.conf` with the following content:

~~~
VAULT=my-backup
~~~

Usage
-----

### Upload an archive

~~~
agar up <directory> <category> <password reminder>
~~~

If a file named `~/.agar/<password reminder>` exists then its contents will 
be used as the password and no password prompt will be issued.

### Check archival status of files/directories

To list the items in a given directory that need to be archived because they
either do not exist in the archive or have been updated since the last archival:

~~~
agar status <directory>
~~~

### Retrieve index

To start the job:

~~~
agar index start
~~~

To check job status and retrieve the index if the job completed:

~~~
agar index get
~~~


License
-------

Apache License 2.0
