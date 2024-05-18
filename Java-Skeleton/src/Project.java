public class Project {
    
	int id; // IDs are 0-based
    int topic; // Topics are 0-based!
    int cap;

    public Project(int ID, int topic, int cap) {
        this.id = ID;
        this.topic = topic;
        this.cap = cap;
    }

    public int getId() {
        return id;
    }

    public int getTopic() {
        return topic;
    }

    public int getCap() {
        return cap;
    }
    
}
