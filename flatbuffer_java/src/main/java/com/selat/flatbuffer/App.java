package com.selat.flatbuffer;

import MyGame.Sample.*;

import java.io.IOException;
import java.nio.ByteBuffer;
import java.nio.file.*;

public class App {

    public static void main(String[] args) throws IOException {

        // Read file content
        final byte[] fileContent = Files.readAllBytes(Path.of("/tmp/flatbuffer.dat"));

        // Create bytebuffer
        final ByteBuffer buf = ByteBuffer.wrap(fileContent);

        // Get an accessor to the root object inside the buffer
        Monster monster = Monster.getRootAsMonster(buf);

        short hp = monster.hp();
        short mana = monster.mana();
        String name = monster.name();

        System.out.printf("HP: %d, Mana: %d, Name: %s\n", hp, mana, name);

        int unionType = monster.equippedType();
        if (unionType == Equipment.Weapon){
            Weapon weapon = (Weapon)monster.equipped(new Weapon());
            String weaponName = weapon.name();
            short weaponDamage = weapon.damage();
            System.out.printf("Weapon Name: %s, Weapon Damage: %s\n", weaponName, weaponDamage);
        }
    }
}
