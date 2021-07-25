using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using LeicaD.Web.Data;
using LeicaD.Web.Models;
using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;

namespace LeicaD.Web.Services
{
    public class AdminUserService : IAdminUserService
    {
        private readonly ApplicationDbContext _db;
        private readonly UserManager<ApplicationUser> _userManager;
        private readonly RoleManager<IdentityRole> _roleManager;

        public AdminUserService(ApplicationDbContext dbContext, UserManager<ApplicationUser> userManager, RoleManager<IdentityRole> roleManager)
        {
            _db = dbContext;
            _userManager = userManager;
            _roleManager = roleManager;
        }
        public async Task<User[]> GetUsersAsync()
        {

            // var newRole = "Admin";

            // var user2 = _userManager.FindByNameAsync("baum@bla.de").Result;

            // //remove all old roles first (if any)
            // var oldRoles = _userManager.GetRolesAsync(user2).Result;
            // foreach (var role in oldRoles)
            // {
            //     await _userManager.RemoveFromRoleAsync(user2, role);
            // }

            // await _userManager.AddToRoleAsync(user2, newRole);



            List<User> usersDto = new List<User>();

            var users = await _db.Users.ToListAsync();

            foreach (var user in users)
            {
                User u = new User();
                u.Username = user.UserName;

                var roles = _userManager.GetRolesAsync(user).Result;
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
            return await _db.Roles.ToArrayAsync();
        }

        public async Task<bool> UpdateUserRoleAsync(User u, string newRole)
        {
            var user = _userManager.FindByNameAsync(u.Username).Result;

            //remove all old roles first (if any)
            var oldRoles = _userManager.GetRolesAsync(user).Result;
            foreach (var role in oldRoles)
            {
                await _userManager.RemoveFromRoleAsync(user, role);
            }

            await _userManager.AddToRoleAsync(user, newRole);

            return true;
        }
    }
}
