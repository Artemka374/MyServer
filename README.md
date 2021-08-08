## Table of contents
* [General info](#general-info)
* [Technologies](#technologies)
* [Usage](#usage)

## General info
Application that allows store notes' names and text in the cloud and search through them.
## Technologies
Project created with:
* Rust

## Requirements
* Postgres
* Cargo

## Usage
### How to build
```
sudo -i -u postgres
```
```
createdb my_db
```
```
psql -d my_db
```
```
CREATE TABLE notes(
   id SERIAL PRIMARY KEY,
   name VARCHAR NOT NULL,
   text VARCHAR NOT NULL
);
```
### Start
`cargo run`

### Endpoints
The program designed as a rest api.
#### Upload
`POST /file`
```json
{
   "name": "file_name.ext",
   "text" : "text"
}
```
as a result of success you will receive status 200 and body:
```json
{
   "ID": "unique file ID"
}
```
#### Delete file
`DELETE  /file/{ID}`

If success it returns status 200 and body:
```json
{"success": "ID of deleted note"}
```
#### Update file
`PUT /file/{ID}`
```json
{
   "name": "file_name.ext",
   "text" : "text"
}
```
as a result of success you will receive status 200 and body:
```json
{
   "ID": "unique file ID"
   "name": "file_name.ext",
   "text" : "text"
}
```
#### List files with pagination optionally filtered by filename
`GET /file?&page=2&size=3`
Here:
* page - [optional] the 0-based parameter for paging. If not provided use 0 (the first page)
* size - [optional] the page size parameter. If not passed use default value 10
* q - [optional] the filename substring parameter

Example of result:
```json
{
   {
      "id": "ID1",
      "name": "note1",
      "text": "abc"
   },
   {
      "id": "ID2",
      "name": "note2",
      "size": "def"
   },
   {
      "id": "ID3",
      "name": "note3",
      "size": "ghi"
   }
}

```