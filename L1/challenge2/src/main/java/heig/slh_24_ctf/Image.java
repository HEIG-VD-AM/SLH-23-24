package heig.slh_24_ctf;

import lombok.Getter;
import lombok.Setter;

import javax.imageio.ImageIO;
import java.awt.*;
import java.awt.image.BufferedImage;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.Base64;

@Getter
public class Image {
    @Setter
    private int brightness;

    @Getter
    @Setter
    private String image;

    @Getter
    @Setter
    private String comment;

    public String convert() throws IOException {
        BufferedImage originalImage = decodeBase64ToImage(this.image);
        BufferedImage blackAndWhiteImage = convertToBlackAndWhite(originalImage);
        BufferedImage adjustedImage = adjustBrightness(blackAndWhiteImage, this.brightness);
        return encodeImageToBase64(adjustedImage);

    }

    public static BufferedImage decodeBase64ToImage(String imageString) throws IOException {
        byte[] imageBytes = Base64.getDecoder().decode(imageString);
        return ImageIO.read(new ByteArrayInputStream(imageBytes));
    }

    public static String encodeImageToBase64(BufferedImage image) throws IOException {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        ImageIO.write(image, "jpg", bos);
        byte[] imageBytes = bos.toByteArray();
        return Base64.getEncoder().encodeToString(imageBytes);
    }


    private static BufferedImage convertToBlackAndWhite(BufferedImage image) {
        BufferedImage resultImage = new BufferedImage(image.getWidth(), image.getHeight(), BufferedImage.TYPE_BYTE_GRAY);
        resultImage.getGraphics().drawImage(image, 0, 0, null);
        return resultImage;
    }

    private static BufferedImage adjustBrightness(BufferedImage image, int brightnessRatio) {
        float ratio = brightnessRatio / 100.0f;
        BufferedImage adjustedImage = new BufferedImage(image.getWidth(), image.getHeight(), BufferedImage.TYPE_BYTE_GRAY);

        for (int y = 0; y < image.getHeight(); y++) {
            for (int x = 0; x < image.getWidth(); x++) {
                Color color = new Color(image.getRGB(x, y));
                int grayValue = color.getRed(); // Assuming it's a grayscale image
                int newGrayValue = (int) (grayValue * ratio);
                newGrayValue = Math.min(255, Math.max(0, newGrayValue)); // Clamp values
                Color newColor = new Color(newGrayValue, newGrayValue, newGrayValue);
                adjustedImage.setRGB(x, y, newColor.getRGB());
            }
        }
        return adjustedImage;
    }
}
