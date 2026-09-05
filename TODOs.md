## TODOs:

### Disk storage
- [ ] enable disk to handle file and pages count as document type.
- [ ] try to extend the tracking of allocated page with page id in the pages.
- [ ] fsync database file to persist in db file.
- [ ] test the disk storage by using temporary file buffer.

### Page format
- [ ] use little endian reads and writes.
- [ ] page should consists of header with page no with the body having records.
- [ ] slotted page should grow from front, records from back. slotted page should have offset + length. 
- [ ] inspect compaction.
