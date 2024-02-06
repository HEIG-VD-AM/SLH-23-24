package heig.slh_24_ctf;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertDoesNotThrow;
import static org.junit.jupiter.api.Assertions.assertThrows;

class CommandFileGeneratorTest {

    @Test
    void testValidInput() {
        assertDoesNotThrow(() -> CommandFileGenerator.verifyCommandParameters("."));
        assertDoesNotThrow(() -> CommandFileGenerator.verifyCommandParameters("/"));
        assertDoesNotThrow(() -> CommandFileGenerator.verifyCommandParameters("abc"));
        assertDoesNotThrow(() -> CommandFileGenerator.verifyCommandParameters("a/b"));
        assertDoesNotThrow(() -> CommandFileGenerator.verifyCommandParameters("./abc"));
    }

    @Test
    void testInvalidInput() {
        assertThrows(IllegalArgumentException.class, () -> CommandFileGenerator.verifyCommandParameters(".."));
        assertThrows(IllegalArgumentException.class, () -> CommandFileGenerator.verifyCommandParameters("../"));
        assertThrows(IllegalArgumentException.class, () -> CommandFileGenerator.verifyCommandParameters("abc/.."));
        assertThrows(IllegalArgumentException.class, () -> CommandFileGenerator.verifyCommandParameters("/.."));
        assertThrows(IllegalArgumentException.class, () -> CommandFileGenerator.verifyCommandParameters("a/../b"));
    }
}
