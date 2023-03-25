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

### Download an archive

~~~
agar down start <archive-id-1> <archive-id-2> ...
~~~

The archive ids can be found in the index file, `~/.agar/index`. If the index file is missing or incomplete then please see the next section for how to reconstruct it.

To check job status and, if ready, retrieve, decrypt and unpack the archive:

~~~
agar down get
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

### Stored passwords

In order to not have to type in the encryption/decryption password, you can pick a password reminder; for instance, if the password is [`correct horse battery staple`](https://xkcd.com/936/), you can use `xkcd-936` as a reminder. You can then store this password in a file called `~/.agar/xkcd-936`, and if `xkcd-936` is specified as the password reminder when uploading, Agar will use the contents of that file as the password. Similarly, when downloading, Agar will look up the password reminder in the index file and will use the contents of a corresponding password file (if present) as the password when decrypting the archive.


License
-------

Apache License 2.0
