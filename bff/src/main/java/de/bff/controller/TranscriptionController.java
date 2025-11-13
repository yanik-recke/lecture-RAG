package de.bff.controller;

import de.bff.service.LectureService;
import lectureservice.LectureServiceOuterClass;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api/v1/transcription")
public class TranscriptionController {

    private static final Logger log = LoggerFactory.getLogger(TranscriptionController.class);

    private final LectureService lectureService;

    public TranscriptionController(LectureService lectureService) {
        this.lectureService = lectureService;
    }

    @PostMapping(consumes = MediaType.MULTIPART_FORM_DATA_VALUE)
    public ResponseEntity<Map<String, String>> transcribe(
            @RequestParam("file") MultipartFile file,
            @RequestParam("lectureName") String lectureName,
            @RequestParam("module") String module) {

        log.info("Received transcription request for lecture: {} in module: {}", lectureName, module);

        try {
            byte[] audioData = file.getBytes();
            String fileName = file.getOriginalFilename();

            LectureServiceOuterClass.TranscribeRes response = lectureService.transcribe(
                    audioData, fileName, lectureName, module);

            Map<String, String> result = new HashMap<>();

            if (response.hasErrorMessage()) {
                log.error("Transcription failed: {}", response.getErrorMessage());
                result.put("error", response.getErrorMessage());
                return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(result);
            }

            result.put("message", "Transcription completed successfully");
            return ResponseEntity.ok(result);

        } catch (IOException e) {
            log.error("Failed to read file: {}", e.getMessage(), e);
            Map<String, String> error = new HashMap<>();
            error.put("error", "Failed to read file: " + e.getMessage());
            return ResponseEntity.status(HttpStatus.BAD_REQUEST).body(error);
        } catch (RuntimeException e) {
            log.error("Transcription error: {}", e.getMessage(), e);
            Map<String, String> error = new HashMap<>();
            error.put("error", "Transcription failed: " + e.getMessage());
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(error);
        }
    }
}
