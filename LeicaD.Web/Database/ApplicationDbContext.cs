using System;
using System.Collections.Generic;
using System.Text;
using Microsoft.AspNetCore.Identity;
using Microsoft.AspNetCore.Identity.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Data
{
    public class ApplicationDbContext : IdentityDbContext
    {
        public DbSet<KenRQuote> KenRQuotes { get; set; }
        public ApplicationDbContext(DbContextOptions<ApplicationDbContext> options)
            : base(options)
        {
        }
        protected override void OnModelCreating(ModelBuilder modelBuilder)
        {
            base.OnModelCreating(modelBuilder);

            modelBuilder.Entity<IdentityRole>().HasData(new List<IdentityRole> {
                new IdentityRole {
                    Id = "1",
                    Name = "Admin",
                    NormalizedName = "ADMIN"
                },
                new IdentityRole {
                    Id = "2",
                    Name = "Mod",
                    NormalizedName = "MOD"
                },
                new IdentityRole {
                    Id = "3",
                    Name = "BotMod",
                    NormalizedName = "BOTMOD"
                },
                new IdentityRole {
                    Id = "4",
                    Name = "User",
                    NormalizedName = "USER"
                },
                });
        }
    }

    public class KenRQuote
    {
        public int Id { get; set; }
        public string Quote { get; set; }
        public DateTime? LastPosted { get; set; }
    }
}
