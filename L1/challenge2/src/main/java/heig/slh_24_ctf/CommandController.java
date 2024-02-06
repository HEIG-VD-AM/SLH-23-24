package heig.slh_24_ctf;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import java.io.FileNotFoundException;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;

/**
 * Controller to read a command stored in a File.txt
 */
@Controller
public class CommandController {

    private static final Logger logger = LoggerFactory.getLogger(CommandController.class);

    @GetMapping("/command/{name}")
    public String readCommand(@PathVariable String name, Model model) {
        String externalDirectoryPath = "/generated/";

        // Build the full file path
        String filePath = externalDirectoryPath + name + ".txt";

        // Create a File object for the file
        File file = new File(filePath);

        try (InputStream inputStream = new FileInputStream(file)) {
            // Read the content from the file
            String content = new String(inputStream.readAllBytes(), StandardCharsets.UTF_8);

            // Log that the file has been successfully read
            logger.info("File {} has been read", name);

            Files.delete(file.toPath());
            logger.info("File {} has been deleted", name);

            // Add the content to the model for rendering in the view
            model.addAttribute("content", content);

            return "command";
        } catch (FileNotFoundException e) {
            // Handle the case where the file does not exist
            logger.warn("File {} does not exist", name);
            return "404";
        } catch (IOException e) {
            // Handle other IO exceptions
            logger.error("Error reading file {} - {}", name, e.getMessage());
            return "error"; // You can create an error page for such cases
        }
    }
}

