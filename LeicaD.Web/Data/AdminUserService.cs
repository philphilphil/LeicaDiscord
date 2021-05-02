using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Data
{
    [Authorize(Roles = "Admin")]
    public class AdminUserService
    {
        private IDbContextFactory<ApplicationDbContext> DbFactory { get; set; }
        private readonly UserManager<IdentityUser> UserManager;
        private readonly RoleManager<IdentityRole> RoleManager;

        public AdminUserService(IDbContextFactory<ApplicationDbContext> _dbFactory, UserManager<IdentityUser> _userManager, RoleManager<IdentityRole> _roleManager)
        {
            DbFactory = _dbFactory;
            UserManager = _userManager;
            RoleManager = _roleManager;
        }
        public async Task<User[]> GetUsersAsync()
        {
            List<User> usersDto = new List<User>();
            using var db = DbFactory.CreateDbContext();
            var users = db.Users.ToList();

            foreach (var user in users)
            {
                User u = new User();
                u.Username = user.UserName;

                var roles = UserManager.GetRolesAsync(user).Result;
                if (roles.Count() == 1)
                {
                    u.Role = roles[0];
                }
                else
                {
                    u.Role = "no role";
                }
                usersDto.Add(u);
            }
            return usersDto.ToArray();
        }

        public async Task<IdentityRole[]> GetRolesAsync()
        {
            using var db = DbFactory.CreateDbContext();
            return await db.Roles.ToArrayAsync();
        }

        public async Task<bool> UpdateUserRoleAsync(User u, string newRole)
        {
            var user = UserManager.FindByNameAsync(u.Username).Result;

            //remove all old roles first (if any)
            var oldRoles = UserManager.GetRolesAsync(user).Result;
            foreach (var role in oldRoles)
            {
                await UserManager.RemoveFromRoleAsync(user, role);
            }

            await UserManager.AddToRoleAsync(user, newRole);

            return true;
        }
    }
}
