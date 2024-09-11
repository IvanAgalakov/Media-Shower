## Simple Media Viewer
A very simple Media Viewer **(with no interface)** that loads both Pictures and Video from its `/media` folder, it loads the media based on the filenames within the folders.

### Quick Start
Download the latest release zips from the releases page:
https://github.com/IvanAgalakov/media-viewer/releases


Put your media within the `/media` folder, all common image types should be supported.

Your input keypresses are recorded, and when you type out the name of a file or folder in the media folder, it is displayed.

For a folder containing frames of a video you need to display, you are required to have a `info.json` file present there, here is an example:
```json
{
    "framerate": 24,
    "frame_extension": "jpg",
    "frame_num": 250
}
```
You must specify the framerate of the video in `"framerate"`
The extension of the frames without a . in `"frame_extension"`
The number of frames in the folder in `"frame_num"`

### WIP
- ffmpeg suppport, however ffmpeg will not be included in the binary, and must be installed seperately.
- qstreamer (got to check how much easier this would be)
- bugfixes and etc.
