# tags-backend
This is my CLI for my file tagging project. It handles the actual mutating of the file that contains all of your tags.

# How to use
### User Mode
#### Setup mode
Preforms first time setup
Should create a **.tags_meta** and **tags** file
.tags_meta will hold the tags files path.
tags file will hold your tags
```
-S
```
if they move **tags**, they will have to update the .tags_meta file
#### Creating tag
(create tag)

```
-ct {tag}
```
-ct test
#### Adding tags to file
(add tag)

```
-at {path} {tag} ... {tag}
```
-at /path/to/file tag1 tag2 tag3 ... etc
#### Removing tags from file
(remove tag)

```
-rt {path} {tag} ... {tag}
```
-rt /path/to/file tag1 tag2 tag3 ... etc

#### Remove ALL tags from files
(un-tag)

```
-ut {path}
```
-ut /path/to/file

---
### Frontend Mode
Frontend mode is ONLY for returning the filesystem to a frontend handler.
#### Give Frontend
(Frontend)
The basic frontend. no filtering, just every file.
```
-F
```
