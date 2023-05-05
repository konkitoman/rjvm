public class Main{
    int a = 1;
    int b = 2;
    
    public Main(int a, int b){
        this.a = a;
        this.b = b;
    }

    public static void main(String[] args){
        Main main1 = new Main(21, 30);
        int a = main1.a + main1.b;
        Main main2 = new Main(5, -4);
        int b = main2.a + main2.b;
    }

}

