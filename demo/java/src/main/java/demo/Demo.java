package demo;
import dotjyphen.Translate;

public class Demo {
  private static Translate translate = new Translate();

  public static void main(String[] args)  {
    System.out.println(translate.intoMorse("Hello World"));
  }
}