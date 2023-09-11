package dotjyphen;

public class Translate {
  static {
      System.loadLibrary("dotjyphen");
  }

  private static native String jniTranslate(String input);

  public static String intoMorse (String input) {
      return jniTranslate(input);
  }
}