package de.bff.service;

import com.google.protobuf.ByteString;
import lectureservice.LectureServiceGrpc;
import lectureservice.LectureServiceOuterClass;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Service;

@Service
public class LectureService {

    private static final Logger log = LoggerFactory.getLogger(LectureService.class);

    private final LectureServiceGrpc.LectureServiceBlockingStub lectureServiceStub;

    public LectureService(LectureServiceGrpc.LectureServiceBlockingStub lectureServiceStub) {
        this.lectureServiceStub = lectureServiceStub;
    }

    public LectureServiceOuterClass.TranscribeRes transcribe(byte[] audioData, String fileName, String lectureName, String module) {
        log.info("Transcribing audio file: {} for lecture: {} in module: {}", fileName, lectureName, module);

        LectureServiceOuterClass.AudioFile audioFile = LectureServiceOuterClass.AudioFile.newBuilder()
                .setAudioData(ByteString.copyFrom(audioData))
                .setFileName(fileName)
                .build();

        LectureServiceOuterClass.TranscribeReq request = LectureServiceOuterClass.TranscribeReq.newBuilder()
                .setFilePayload(audioFile)
                .setLectureName(lectureName)
                .setModule(module)
                .build();

        try {
            LectureServiceOuterClass.TranscribeRes response = lectureServiceStub.transcribe(request);
            log.info("Transcription completed successfully");
            return response;
        } catch (Exception e) {
            log.error("Error during transcription: {}", e.getMessage(), e);
            throw new RuntimeException("Failed to transcribe audio", e);
        }
    }

    public LectureServiceOuterClass.SearchRes search(String prompt, String module) {
        log.info("Searching with prompt: {}", prompt);

        LectureServiceOuterClass.Prompt promptPayload = LectureServiceOuterClass.Prompt.newBuilder()
                .setPrompt(prompt)
                .build();

        LectureServiceOuterClass.SearchReq request = LectureServiceOuterClass.SearchReq.newBuilder()
                .setModule(module)
                .setPromptPayload(promptPayload)
                .build();

        try {
            LectureServiceOuterClass.SearchRes response = lectureServiceStub.search(request);
            log.info("Search completed successfully");
            return response;
        } catch (Exception e) {
            log.error("Error during search: {}", e.getMessage(), e);
            throw new RuntimeException("Failed to search", e);
        }
    }
}
