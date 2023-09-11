package dotjyphen;

import static org.junit.Assert.*;
import org.junit.Test;

public class TranslateTest {
  
  private Translate translate = new Translate();

  @Test
  public void translateHelloWorld() {
    assertEquals(translate.intoMorse("Hello World"), ".... . .-.. .-.. --- / .-- --- .-. .-.. -..");
  }

}