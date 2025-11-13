package de.bff.controller;

import de.bff.service.LectureService;
import lectureservice.LectureServiceOuterClass;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.*;

import java.util.HashMap;
import java.util.Map;

@RestController
@RequestMapping("/api/v1/search")
public class SearchController {

    private static final Logger log = LoggerFactory.getLogger(SearchController.class);

    private final LectureService lectureService;

    public SearchController(LectureService lectureService) {
        this.lectureService = lectureService;
    }

    @PostMapping
    public ResponseEntity<Map<String, String>> search(@RequestBody SearchRequest searchRequest) {
        log.info("Received search request with prompt: {}", searchRequest.getPrompt());

        try {
            LectureServiceOuterClass.SearchRes response = lectureService.search(searchRequest.getPrompt());

            Map<String, String> result = new HashMap<>();

            if (response.hasErrorMessage()) {
                log.error("Search failed: {}", response.getErrorMessage());
                result.put("error", response.getErrorMessage());
                return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(result);
            }

            result.put("message", "Search completed successfully");
            return ResponseEntity.ok(result);

        } catch (RuntimeException e) {
            log.error("Search error: {}", e.getMessage(), e);
            Map<String, String> error = new HashMap<>();
            error.put("error", "Search failed: " + e.getMessage());
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(error);
        }
    }

    public static class SearchRequest {
        private String prompt;

        public SearchRequest() {
        }

        public SearchRequest(String prompt) {
            this.prompt = prompt;
        }

        public String getPrompt() {
            return prompt;
        }

        public void setPrompt(String prompt) {
            this.prompt = prompt;
        }
    }
}
