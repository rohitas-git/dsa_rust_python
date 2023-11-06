# Live streaming system

## Requirements
1. Streaming video
2. Processing video
3. Broadcasting [CORE]
4. Failproof
5. Advertisements
6. Reactions
7. Disclaimer/News flashes
8. Degradation of video quality 
9. Multiple device support 

Capture Video from a source -> Store it in someplace in my server to query later(milliseconds in Live streaming) -> Distribute to the users from storage -> Queried through APIs by user to access video

## APIs

These APIs have well defined signatures. 
GET /video  -> VideoFrames

APIs not queried through programming langauge but through a network protocol like grpc, http, ftp ...
i.e any kind of protocol which defines exactly 
- how an electronic message is going to be taken from one place to another
- how the response is going to come back 
- how the behavior of this interaction is going to defined by the protocol


## Failure scenarios
As engineers, we have to think of various failures scenarios.

Ex:
- database storing videos, crashes
- a particular firewalls on the internet starts blocking of your requests
- if one piece of code in your software starts misbehaving due to bug/attacker 
- feature request by user (say allowing artist to talk with a audience member)


# Designing it

Two ways:
1. Customers -> Server -> Database
Customer define their problems,
which are then fulfilled using APIs on server,
which are then fulfilled by storing some sort of data in the database 

2. Database -> Server -> Customer
What kind of data I need to store to enable my server? 
What kind of apis needed to expose to enable my customers to use the product?

Both approaches are fine and req diff ways of thinking 

## 1st approach

users use multiple device to live stream -> frontend UI design problem

system design -> distributed system backend part of things

## High level view / Rough design
Client 

->  Server
- API:
  - 1. Get VideoFrame(id, device, offset) -> Frame 
  - 2. Post Comment(id, data, author, video_id) 

-> Db
  - Video table
  - User table
  - Comment table  

## More Implementation details

### Client - Server side

2. - continuous notification not required  => HTTP protocol  (most common protocol for distributed systems)

HTTP protocol benefit:
- stateless server
  - no need to store any information while handling a request
  - No need to know where you are from, what you want. 
  - Define everything (total context) in the request itself


1. 
- continuous notification required => GET Video, use webRTC / MPEG DASH (p2p protocol)
    HTTP can also be used 
    but a better protocol would be specific to video transmission 

    TCP <= Reliable protocol
    UDP <= Real time efficient protocol

    webRTC <= for video conferences 
    MPEG DASH <= Broadcasting  (dynamic adaptive streaming over HTTP)
    HLS <= similar to MPEG DASH but for Mac iOS devices 

- Record Video in storage by HD Camera: 
    Network protocol => PUT Video, use RTMP (Real time Media protocol)

### Server - DB side

Database solutions define how exactly the client interacts with the database
Protocol is well defined for them

which database to use?
Tons of Db options out there: Tradeoffs are there

Video is a file. We can use File system for db. 
For VIDEO table:: Ex: HDFS, Amazon S3, Vimeo?? ... 
   - Cheap
   - Easy to query
   - Store very large files 
  
For User :: MySQL / Postgresql
For comment table:: NoSQL db


## Nitty gritties of designing it

Raw Video (8k) will not be sent to customers 

It passed to transformation software for multiple video quality transformer
- 4k
- 1080p
- 720p
- 480p
- 144p

There may also be another transformation according to device OS's diff media support
Common video format - H264

How to do it? 
- Break raw video into 10sec segemnts and passed to each transformer program 
- and get combination of resolution and format converted from raw video  

Map reduce pattern is used to convert raw video into data streams for storage.

MPEG DASH is suitable for broadcasting. Adapts video resolution according to bandwidth.

Server should also cache last 10 mins of video (instead of making call to db, saving both time and bandwidth)

We also need fault tolerance here. => use CDN solutions (Content delivery networks)
to persist some static data (like webpages and small video data)

## Important things to notice

High level blueprint:
1. Define the requirements as abstract concepts (objects)
2. Objects need to be able to manipulated and queried using API's on server
3. The data representation needs to be stored in databases.

Then think how to make this possible:
- Various Network protocols
- Db solutions
- what kind of design patterns will be useful

Finally, after deciding the tools:
- Think of iteractions between these tools
- and interactions of these services to meet our requirements


# Low level Design

The core functionality of live streaming is to be able to view a video as a customer.

How does a user fetch video, view video and fetch more of it till video ends?
- Fetch video
- View video

Two approaches:
1. One is to think of code in the start
2. Think of user's possible actions
   - Play video at timestamp [x]
   - Pause the video
   - quality of video acc. to device (adaptive video quality)
   - Last watched seek position of already watched video
   - Non stop play/buffer when watching videos


Considerations:
- memory optimizations
- user behaviour
- api calling

Different low level requirements affect the code we write

Tools to have a strucuted approach to solving problem
- **use case diagram**
    Think all the actors in the system
    - Customer's use cases:
        - Play video at timestamp [x]
        - Pause the video
        - quality of video acc. to device (adaptive video quality)
        - Last watched seek position of already watched video
        - Non stop play/buffer when watching videos

- **class diagram** comes up on the foundation of use case diagram 
  - clearer the apis, the easier it will be to come up with low lvl design
  - think about each feature is going impl using the services that you have 

Class : - State     - Behaviours

States are data that object needs to perform behaviours

Video:
    - Id
    - Bytes
    - Metadata
    * getFrame()

User:
    - Id
    - Name
    - Email
    * getId()

WatchedVideo:
    - Id
    - video_id
    - user_id
    - seek_timestamp
    * getId()

VideoConsumingService:
    - WatchedVideo[]
    * getVideoFrame(userId, videoId, timestamp)
    * seek(user_id,video_id)

- **Sequence Diagram**
- 