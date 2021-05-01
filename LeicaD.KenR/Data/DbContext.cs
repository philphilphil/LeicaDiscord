//using Microsoft.EntityFrameworkCore;
//using System;
//using System.Collections.Generic;
//using System.Text;

//namespace KenR_LeicaBot.Data
//{
//    public class KenContext : DbContext
//    {
//        public DbSet<DynamicTextCommand> DynamicTextCommands { get; set; }

//        protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
//        {
//            optionsBuilder.UseSqlite("Filename=KenR.db");
//        }
//    }

//    public class DynamicTextCommand
//    {
//        public int DynamicTextCommandId { get; set; }
//        public string Command { get; set; }
//        public string Response { get; set; }
//        public string CreatedBy { get; set; }
//    }
//}