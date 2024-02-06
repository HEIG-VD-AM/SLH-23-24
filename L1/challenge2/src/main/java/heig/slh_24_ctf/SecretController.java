package heig.slh_24_ctf;

import org.springframework.beans.factory.annotation.Value;
import org.springframework.stereotype.Controller;
import org.springframework.ui.Model;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestParam;

@Controller
public class SecretController {

    @Value("${secret.data}")
    private String secretData; // Read the encrypted data from application.properties

    @GetMapping("/secret")
    public String getLoginPage() {
        return "secret-form";
    }

    @PostMapping("/secret")
    public String decryptData(@RequestParam String secret, Model model) {
        try {
            String decryptedData = CryptoUtils.decrypt(secretData, secret);
            model.addAttribute("decryptedData", decryptedData);
            return "secret-result"; // Return a Thymeleaf template to display the decrypted data
        } catch (Exception e) {
            model.addAttribute("error", "Error decrypting data");
            return "secret-form"; // Return to the form with an error message
        }
    }
}
