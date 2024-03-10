import java.util.*;
import java.lang.*;



class Codec {
    private HashMap<String, String> map = new HashMap<>();

    // Encodes a URL to a shortened URL.
    public String encode(String longUrl) {
        String shortUrl = "http://tinyurl.com/" + longUrl.hashCode();
        map.put(shortUrl, longUrl);
        return shortUrl;
    }

    // Decodes a shortened URL to its original URL.
    public String decode(String shortUrl) {
        // parse the shortUrl to get the key
        return map.get(shortUrl);
    }
}

public class Main {

    public static void main(String[] args) {

    }
}