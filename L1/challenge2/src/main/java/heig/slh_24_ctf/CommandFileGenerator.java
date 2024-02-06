package heig.slh_24_ctf;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.*;
import java.util.List;
import java.util.Vector;
import java.util.regex.Pattern;

public class CommandFileGenerator {
    private static final Logger logger = LoggerFactory.getLogger(CommandFileGenerator.class);
    private final String command;
    private final String fileName;

    public CommandFileGenerator(String fileName, String command) {
        this.command = command;
        this.fileName = fileName;
        try{
            verifyNameParameters(fileName);
            executeCommandAndGenerateFile();
            logger.info("{} command had been successfully executed and stored at /generated/{}", this.command, this.fileName);
        }
        catch (Exception e) {
            logger.warn("{} command failed to execute, or the result could not be stored", command);
        }
    }

    private void executeCommandAndGenerateFile() throws IOException, InterruptedException {
        Process process = getProcess();
        int exitCode = process.waitFor();

        if (exitCode == 0) {
            try (InputStream inputStream = process.getInputStream();
                 InputStreamReader inputStreamReader = new InputStreamReader(inputStream);
                 BufferedReader bufferedReader = new BufferedReader(inputStreamReader)) {

                StringBuilder output = new StringBuilder();
                String line;
                while ((line = bufferedReader.readLine()) != null) {
                    output.append(line).append(System.lineSeparator()); // Preserve newline
                }
                generateAndSaveHtmlFile(output.toString());
            }
        } else {
            throw new IOException("Command execution failed with exit code: " + exitCode);
        }
    }

    private void generateAndSaveHtmlFile(String content) throws IOException {
        String storagePath = "/generated";
        File directory = new File(storagePath);
        if (!directory.exists()) {
            directory.mkdirs();
        }

        // Create the file
        File file = new File(directory, this.fileName + ".txt");

        // Write content to the file
        try (OutputStream outputStream = new FileOutputStream(file)) {
            byte[] bytes = content.getBytes();
            outputStream.write(bytes);
        }
        logger.info("Command output saved with name {}", fileName);

    }

    private Process getProcess() throws IOException {
        Vector<String> VALID_COMMAND = new Vector<>(List.of(new String[]{"cat", "ls"}));
        String[] commands = command.split("\\s+");
        if (commands.length > 2 || commands.length == 0){
            logger.warn("{} command should have maximum two params", command);
            throw new IllegalArgumentException("A command has two parameters at maximum");
        }
        if (! VALID_COMMAND.contains(commands[0])) {
            logger.warn("{} command is not allowed", command);
            throw new IllegalArgumentException("This command is not allowed");
        }
        if (commands.length == 2) {
            verifyCommandParameters(commands[1]);
        }

        ProcessBuilder processBuilder = new ProcessBuilder(commands);
        processBuilder.redirectErrorStream(true); // Redirect error stream to the input stream
        return processBuilder.start();
    }
    private static void verifyNameParameters(String name) throws IllegalArgumentException{
        Pattern pattern = Pattern.compile("^[A-Za-z]+$");
        if (!pattern.matcher(name).find() || name.length() < 8){
            logger.warn("{} command name is not valid", name);
            throw new IllegalArgumentException("The name of a command must be compose of a minimum of 8 letters");
        }
    }
    public static void verifyCommandParameters(String command) throws IllegalArgumentException{
        // Only path above
        Pattern pattern = Pattern.compile("^(?!.*\\.{2})[A-Za-z._/]+$");
        if (!pattern.matcher(command).find()){
            logger.warn("{} command parameter is not valid", command);
            throw new IllegalArgumentException("Invalid characters");
        }
    }
}
