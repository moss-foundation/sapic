use joinerror::errors;

errors! {
    /// To avoid overwriting local changes, merge and fast-forward cannot proceed when the worktree
    /// is empty. The frontend should prompt the user to stash local changes before further action.
    DirtyWorktree => "dirty_worktree",

    /// Our app is not able to handle conflict resolution at the moment, thus merges that result in
    /// conflicts will be rejected
    Conflicts => "conflicts",

}
