using LeicaD.Web.Models;
using IdentityServer4.EntityFramework.Options;
using Microsoft.AspNetCore.ApiAuthorization.IdentityServer;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Options;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Identity;

namespace LeicaD.Web.Data
{
    public class ApplicationDbContext : ApiAuthorizationDbContext<ApplicationUser>
    {

        public DbSet<KenRQuote> KenRQuotes { get; set; }

        public ApplicationDbContext(
            DbContextOptions options,
            IOptions<OperationalStoreOptions> operationalStoreOptions) : base(options, operationalStoreOptions)
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
