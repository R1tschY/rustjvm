package de.richardliebscher.rustjvm;

public class ArrayTest {

    public static Object assignNullArray(Object[] arr, Object i) {
        return arr[(int)i];
    }

    public static void main(String[] args) {
        Object[] arr = new Integer[] { 1 };
        assignNullArray(arr, 0);
    }
}
