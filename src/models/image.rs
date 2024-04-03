use chrono::Utc;
use serde::{Deserialize, Serialize};
use wither::bson::oid::ObjectId;

pub struct Image {
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub link_id: String,
    pub timestamp: Utc,
    pub latest_accessed: Utc,
}
// {
//     "imageName": "example_image.jpg",
//     "userId": "user123",
//     "gridFsLinkId": "606e530d13fe601b17aefbe3",
//     "timestamp": "2023-03-09T12:34:56Z",
//     "latestAccessed": "2023-03-10T08:00:00Z"
//   }

//   {
//     "_id": ObjectId("606e530d13fe601b17aefbe3"),
//     "filename": "example_image.jpg",
//     "uploadDate": ISODate("2023-03-09T12:34:56Z"),
//     "length": 123456,
//     "metadata": {
//       "userId": "user123",
//       "timestamp": "2023-03-09T12:34:56Z",
//       "latestAccessed": "2023-03-10T08:00:00Z"
//     },
//     "contentType": "image/jpeg",
//     "chunkSize": 261120,
//     "md5": "d41d8cd98f00b204e9800998ecf8427e"
//   }
