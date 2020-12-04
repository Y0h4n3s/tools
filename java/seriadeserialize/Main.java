package seriadeserialize;

import jdk.jshell.spi.ExecutionControl;

import java.io.*;
import java.io.Serializable;
import java.lang.reflect.InvocationTargetException;
import java.nio.file.Path;
import java.util.Optional;
import java.util.Base64;

import seriadeserialize._Object;

public class Main {
    public static void main(String[] args) throws ExecutionControl.NotImplementedException, NoSuchMethodException, IllegalAccessException, InvocationTargetException, InstantiationException {
        String command = args[0];
        switch (command) {
            case "serialize" -> {
                System.out.println(serialize(new _Object(10, "test")));
            }
            case "deserialize" -> {
                try(FileInputStream fis = new FileInputStream(Path.of(args[1]).toFile())) {
                    _Object obj = (_Object) deserialize(fis);

                    System.out.println(obj.toString());
                } catch (FileNotFoundException e) {
                    e.printStackTrace();
                } catch (IOException e) {
                    e.printStackTrace();
                }
            }
        }
    }

    private static String serialize(Serializable serializable) {
        String encoded = "";
        try (var baos = new ByteArrayOutputStream()) {
            var oos = new ObjectOutputStream(baos);

            oos.writeObject(serializable);
            oos.flush();
            oos.close();
            encoded = Base64.getEncoder().encodeToString(baos.toByteArray());
        } catch (IOException e) {
            e.printStackTrace();
        }
        return encoded;
    }

    private static <T> T deserialize(FileInputStream fis) {
        T _class = null;
        try (var ois = new ObjectInputStream(fis)) {
            _class = (T) ois.readObject();

        } catch (IOException | ClassNotFoundException e) {
            e.printStackTrace();
        }
        return _class;
    }
}