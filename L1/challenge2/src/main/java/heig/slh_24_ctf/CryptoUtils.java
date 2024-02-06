package heig.slh_24_ctf;

import javax.crypto.Cipher;
import javax.crypto.spec.GCMParameterSpec;
import javax.crypto.spec.SecretKeySpec;
import java.security.Key;
import java.util.Base64;

/**
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * WARNING : NO VULNERABILITES TO SEARCH HERE
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * **/

public class CryptoUtils {
    private static final String ALGORITHM = "AES/GCM/NoPadding";
    private static final int GCM_NONCE_LENGTH = 12; // GCM nonce length in bytes
    private static final int GCM_TAG_LENGTH = 16;  // GCM authentication tag length in bytes

    public static String decrypt(String encryptedData, String secret) throws Exception {
        byte[] encryptedMessage = Base64.getDecoder().decode(encryptedData);
        byte[] nonce = new byte[GCM_NONCE_LENGTH];
        byte[] encryptedDataBytes = new byte[encryptedMessage.length - GCM_NONCE_LENGTH];

        System.arraycopy(encryptedMessage, 0, nonce, 0, GCM_NONCE_LENGTH);
        System.arraycopy(encryptedMessage, GCM_NONCE_LENGTH, encryptedDataBytes, 0, encryptedDataBytes.length);

        Key key = new SecretKeySpec(secret.getBytes(), "AES");
        Cipher cipher = Cipher.getInstance(ALGORITHM);
        GCMParameterSpec gcmSpec = new GCMParameterSpec(GCM_TAG_LENGTH * 8, nonce);
        cipher.init(Cipher.DECRYPT_MODE, key, gcmSpec);

        byte[] decryptedData = cipher.doFinal(encryptedDataBytes);

        return new String(decryptedData);
    }
}
