describe("crud-login", () => {

  it("Register user", async () => {
    await register();
  });

  it("Create post", async () => {
    await createPost();
  });

  it("Update post", async () => {
    await updatePost();
  });

  it("Delete post", async () => {
    await deletePost();
  });

});