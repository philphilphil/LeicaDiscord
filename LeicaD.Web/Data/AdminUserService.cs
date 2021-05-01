using System;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Data
{
    public class AdminUserService
    {
        private IDbContextFactory<ApplicationDbContext> DbFactory { get; set; }
        public AdminUserService(IDbContextFactory<ApplicationDbContext> _dbFactory)
        {
            DbFactory = _dbFactory;
        }
        public Task<User[]> GetUsersAsync()
        {
            using var db = DbFactory.CreateDbContext();

            var users = db.Users.ToList();

            return Task.FromResult(users.Select(index => new User
            {
                Username = index.UserName
            }).ToArray());
        }
    }
}
