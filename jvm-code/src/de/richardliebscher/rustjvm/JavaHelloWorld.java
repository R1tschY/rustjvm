package de.richardliebscher.rustjvm;

public class JavaHelloWorld {
    static long staticInt;
    static Integer staticInteger;

    public static void main(String[] args) {
        long staticInt = JavaHelloWorld.staticInt;
        Integer staticInteger = JavaHelloWorld.staticInteger;

        System.out.println("Hello World!");
    }
}
