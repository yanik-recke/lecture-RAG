# Using OpenAI's API to Create the Transcriptions

## Generate Audio Files

Download the video and convert it to audio only.
```shell
ffmpeg -i BK1a_251016.mp4 -t 2250 -ac 1 -ar 16000 -b:a 64k out1.mp3
ffmpeg -i BK1b_251016.mp4 -ss 2250 -ac 1 -ar 16000 -b:a 64k out2.mp3
```
This creates two
parts which is necessary as *OpenAI* only allows
uploads with a size of up to 25MB.