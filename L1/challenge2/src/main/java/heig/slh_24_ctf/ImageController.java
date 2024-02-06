package heig.slh_24_ctf;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.yaml.snakeyaml.Yaml;

import java.io.ByteArrayInputStream;
import java.io.InputStreamReader;
import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;

@org.springframework.stereotype.Controller
public class ImageController {

    @GetMapping("/")
    public String index() {
        return "index";
    }

    @PostMapping(value = "/convert", consumes = "text/plain")
    public ResponseEntity<String> convert(@RequestBody String requestBody) {

        Image result;
        try {
            result = parse(requestBody);
        } catch (Exception e) {
            logger.warn("Failed to deserialize parameters ; Hash : {}", hashInput(requestBody));
            return ResponseEntity.badRequest().body("Invalid parameters");
        }
        if (result.getImage() == null || result.getImage().isEmpty()) {
            logger.info("A user gave an empty image");
            return ResponseEntity.badRequest().body("The image file is mandatory");
        }
        try {
            String imageBase64String = "data:image/png;base64," + result.convert();
            logger.info("Image conversion successful for input data : {}", hashInput(requestBody));
            return ResponseEntity.ok(imageBase64String);
        } catch (Exception e) {
            logger.warn("An error occurred: {} ; Hash : {}", e.getMessage(), hashInput(requestBody));
            return ResponseEntity.badRequest().body("We were not able to process the file as an image.");
        }
    }

    private String hashInput(String input) {
        MessageDigest messageDigest;
        try {
            messageDigest = MessageDigest.getInstance("SHA-256");
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException(e);
        }
        messageDigest.update(input.getBytes());
        return Base64.getEncoder().encodeToString((messageDigest.digest()));
    }

    private Image parse(String yamlEncoded) throws Exception {
        byte[] yml = Base64.getDecoder().decode(yamlEncoded);
        InputStreamReader reader = new InputStreamReader(new ByteArrayInputStream(yml), StandardCharsets.UTF_8);
        return new Yaml().loadAs(reader, Image.class);
    }

    private static final Logger logger = LoggerFactory.getLogger(ImageController.class);
}
