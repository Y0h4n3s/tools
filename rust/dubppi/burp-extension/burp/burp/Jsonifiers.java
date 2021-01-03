package burp;

import org.json.JSONArray;
import org.json.JSONObject;

import java.util.Objects;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

class Jsonifiers {
    private final BurpExtender burpExtender;

    public Jsonifiers(BurpExtender burpExtender) {
        this.burpExtender = burpExtender;
    }

    public JSONObject jsonifyForDomXssSinks(Matcher matches, String hostname, String link_from) {
        JSONObject json = new JSONObject();
        JSONArray data = new JSONArray();
        while (matches.find()) {
            JSONObject sink = new JSONObject();
            sink.put("hostname", hostname);
            sink.put("link_from", link_from);
            sink.put("sink", matches.group());
            data.put(sink);
        }
        if (data.isEmpty()) return new JSONObject("{}");
        json.put("data", data);
        return json;
    }

    public JSONObject jsonifyForMuchData(Matcher matches, String link_from) {
        JSONObject json = new JSONObject();
        JSONArray data = new JSONArray();
        while (matches.find()) {
            JSONObject much_data = new JSONObject();
            Matcher p = Pattern.compile(".?(?:http?|wss|ssh|ftp|file)*://([a-z0-9\\-._~%!$&'()*+,;=]+@)?([a-z0-9\\-._~%]+|\\[[a-z0-9\\-._~%!$&'()*+,;=:]+\\]):([0-9]+)").matcher(matches.group(0));
            int port = p.matches() ? Integer.parseInt(p.group(3)) : (matches.group(2) == "http" ? 80 : 443);
            much_data.put("full_link", Objects.requireNonNullElse(matches.group(0), ""));
            much_data.put("link_only", Objects.requireNonNullElse(matches.group(1), ""));
            much_data.put("protocol", Objects.requireNonNullElse(matches.group(2), ""));
            much_data.put("port", port);
            much_data.put("hostname", Objects.requireNonNullElse(matches.group(3), ""));
            much_data.put("full_path", Objects.requireNonNullElse(matches.group(4), ""));
            much_data.put("path_only", Objects.requireNonNullElse(matches.group(5), ""));
            much_data.put("params", Objects.requireNonNullElse(matches.group(6), ""));
            much_data.put("page_from", link_from);
            data.put(much_data);
        }
        if (data.isEmpty()) return new JSONObject("{}");
        json.put("data", data);
        return json;
    }

    public JSONObject jsonifyForOwnLinks(Matcher matcher, String protocol, int port, String hostname, String link) {
        JSONObject json = new JSONObject();
        JSONArray data = new JSONArray();

        while (matcher.find()) {
            JSONObject own_link = new JSONObject();
            own_link.put("port", port);
            own_link.put("protocol", protocol);
            own_link.put("hostname", hostname);
            own_link.put("path_only", "/" + Objects.requireNonNullElse(matcher.group(5), ""));
            own_link.put("params", Objects.requireNonNullElse(matcher.group(6), ""));
            own_link.put("full_link", protocol + "://" + hostname + ((port == 80 || port == 443) ? "" : ":" + port) + "/" + Objects.requireNonNullElse(matcher.group(3), ""));
            own_link.put("extracted_from",link);
            data.put(own_link);
        }

        if (data.isEmpty()) return new JSONObject("{}");
        json.put("data", data);
        return json;
    }

    private JSONObject jsonifyTemplate(Matcher matcher) {
        JSONObject json = new JSONObject();
        JSONArray data = new JSONArray();

        // customize matches here
        while (matcher.find()) {

        }

        if (data.isEmpty()) return new JSONObject("{}");
        json.put("data", data);
        return json;
    }
}
