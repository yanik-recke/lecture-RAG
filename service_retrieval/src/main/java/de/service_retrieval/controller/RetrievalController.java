package de.service_retrieval.controller;


import de.service_retrieval.model.RetrievalResponseDTO;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
public class RetrievalController {

    @PostMapping("/retrieve")
    public ResponseEntity<RetrievalResponseDTO> retrieve() {

    }

}
