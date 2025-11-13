package de.bff.controller;

import de.bff.model.SearchRequest;
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
    public ResponseEntity<String> search(@RequestBody SearchRequest searchRequest) {
        log.info("Received search request with prompt: {}", searchRequest.prompt());

        try {
            LectureServiceOuterClass.SearchRes response = lectureService.search(searchRequest.prompt());

            if (response.hasErrorMessage()) {
                log.error("Search failed: {}", response.getErrorMessage());
                return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(response.getErrorMessage());
            } else {
                return ResponseEntity.ok(response.getResponse());
            }
        } catch (RuntimeException e) {
            log.error("Search error: {}", e.getMessage(), e);
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body("An error occurred while trying to process the request");
        }
    }

}
