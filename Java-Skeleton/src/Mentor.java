import java.util.ArrayList;

public class Mentor {

    int id; // IDs are 0-based!
    ArrayList<Integer> proficiencies;

    public Mentor(int ID) {
        this.id = ID;
        this.proficiencies = new ArrayList<>();
    }

    public int getId() {
        return id;
    }

    public void addProficiency(int topic) {
        proficiencies.add(topic);
    }

    public boolean isProficient(int topic) {
        return proficiencies.contains(topic);
    }

}
