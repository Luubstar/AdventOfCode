namespace commons{
    class Commons
    {   
        public static string RED         = Console.IsOutputRedirected ? "" : "\x1b[91m";
        public static string GREEN       = Console.IsOutputRedirected ? "" : "\x1b[92m";
        public static string NORMAL      = Console.IsOutputRedirected ? "" : "\x1b[39m";
        
        public static void Main(){
            Console.WriteLine(GREEN + Dia25.Execute() + NORMAL);
        }
        public static string[] ReadLines(string n){return File.ReadAllLines("../../../" + n);}
        public static string ReadInput(string n){return File.ReadAllText("../../../" +  n);}

    }
}