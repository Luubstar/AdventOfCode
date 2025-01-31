namespace commons{
    class GrafoNoDirigo : Grafo
    {         
        public override void AddConnection(string o, string d, int c){
            base.AddConnection(o,d,c);
            base.AddConnection(d,o,c);
        }
    }
}